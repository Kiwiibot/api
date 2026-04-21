#!/usr/bin/env bash
# Usage: ./api.sh <img_key> [image1] [image2] [--option=value] [--flag]
# Examples:
#   ./api.sh speech_bubble image.png
#   ./api.sh speech_bubble image.png bg.png --flip --background=image --big
#   ./api.sh caption image.png --font=impact --text="Hello World"
#   ./api.sh speech_bubble https://example.com/image.png --direction=right

set -euo pipefail

HOST="${API_HOST:-http://localhost:5555}"
OUTPUT="${API_OUTPUT:-output}"

if [ $# -lt 1 ]; then
    echo "Usage: $0 <img_key> [images...] [--option=value] [--flag] [--text=\"...\"]"
    echo ""
    echo "Images can be file paths or URLs."
    echo "Boolean flags like --flip are set to true."
    echo "Options with values use --option=value syntax."
    echo "Texts use --text=\"your text\" (can be repeated)."
    echo ""
    echo "Environment variables:"
    echo "  API_HOST    - Server URL (default: http://localhost:5555)"
    echo "  API_OUTPUT  - Output filename without extension (default: output)"
    exit 1
fi

KEY="$1"
shift

IMAGES=()
TEXTS=()
declare -A OPTIONS

while [ $# -gt 0 ]; do
    case "$1" in
        --text=*)
            TEXTS+=("${1#--text=}")
            ;;
        --*)
            opt="${1#--}"
            if [[ "$opt" == *"="* ]]; then
                key="${opt%%=*}"
                value="${opt#*=}"
                if [[ "$value" == "true" || "$value" == "false" ]]; then
                    OPTIONS["$key"]="$value"
                elif [[ "$value" =~ ^-?[0-9]+$ ]]; then
                    OPTIONS["$key"]="$value"
                elif [[ "$value" =~ ^-?[0-9]*\.[0-9]+$ ]]; then
                    OPTIONS["$key"]="$value"
                else
                    OPTIONS["$key"]="\"$value\""
                fi
            else
                OPTIONS["$opt"]="true"
            fi
            ;;
        *)
            IMAGES+=("$1")
            ;;
    esac
    shift
done

IMAGES_JSON="["
for i in "${!IMAGES[@]}"; do
    [ "$i" -gt 0 ] && IMAGES_JSON+=","
    img="${IMAGES[$i]}"
    name="$(basename "$img")"

    if [[ "$img" == http://* || "$img" == https://* ]]; then
        IMAGES_JSON+="{\"type\":\"url\",\"url\":\"$img\",\"name\":\"$name\"}"
    elif [ -f "$img" ]; then
        abs_path=$(realpath "$img")
        if command -v cygpath &>/dev/null; then
            abs_path=$(cygpath -w "$abs_path")
        fi
        abs_path_escaped=$(echo "$abs_path" | sed 's/\\/\\\\/g')
        IMAGES_JSON+="{\"type\":\"path\",\"path\":\"$abs_path_escaped\",\"name\":\"$name\"}"
    else
        echo "Error: File not found: $img" >&2
        exit 1
    fi
done
IMAGES_JSON+="]"

TEXTS_JSON="["
for i in "${!TEXTS[@]}"; do
    [ "$i" -gt 0 ] && TEXTS_JSON+=","
    escaped=$(echo "${TEXTS[$i]}" | sed 's/\\/\\\\/g; s/"/\\"/g')
    TEXTS_JSON+="\"$escaped\""
done
TEXTS_JSON+="]"

OPTIONS_JSON="{"
first=true
for key in "${!OPTIONS[@]}"; do
    $first || OPTIONS_JSON+=","
    first=false
    OPTIONS_JSON+="\"$key\":${OPTIONS[$key]}"
done
OPTIONS_JSON+="}"

PAYLOAD_FILE=$(mktemp)
RESPONSE_FILE=$(mktemp)
trap "rm -f '$PAYLOAD_FILE' '$RESPONSE_FILE'" EXIT
printf '{"images":%s,"texts":%s,"options":%s}' "$IMAGES_JSON" "$TEXTS_JSON" "$OPTIONS_JSON" > "$PAYLOAD_FILE"

echo "→ POST $HOST/image/$KEY" >&2
echo "  Options: $OPTIONS_JSON" >&2
echo "  Images: ${#IMAGES[@]}, Texts: ${#TEXTS[@]}" >&2

RESPONSE=$(curl -s -w "\n%{http_code}\n%{content_type}" \
    -X POST \
    -H "Content-Type: application/json" \
    -d "@$PAYLOAD_FILE" \
    -o "$RESPONSE_FILE" \
    "$HOST/image/$KEY")

HTTP_CODE=$(echo "$RESPONSE" | head -1)
CONTENT_TYPE=$(echo "$RESPONSE" | tail -1)

if [[ "$CONTENT_TYPE" == *"image/"* ]]; then
    case "$CONTENT_TYPE" in
        *png*)  EXT="png" ;;
        *gif*)  EXT="gif" ;;
        *jpeg*) EXT="jpg" ;;
        *webp*) EXT="webp" ;;
        *)      EXT="png" ;;
    esac
    OUTFILE="${OUTPUT}.${EXT}"
    mv "$RESPONSE_FILE" "$OUTFILE"
    echo "✓ Saved to $OUTFILE" >&2
elif [[ "$CONTENT_TYPE" == *"video/"* ]]; then
    case "$CONTENT_TYPE" in
        *mp4*)  EXT="mp4" ;;
        *)      EXT="mp4" ;;
    esac
    OUTFILE="${OUTPUT}.${EXT}"
    mv "$RESPONSE_FILE" "$OUTFILE"
    echo "✓ Saved to $OUTFILE" >&2
else
    echo "✗ Error (HTTP $HTTP_CODE):" >&2
    cat "$RESPONSE_FILE" >&2
    echo "" >&2
    rm -f "$RESPONSE_FILE"
    exit 1
fi
