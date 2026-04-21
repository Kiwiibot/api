"""
Scans all homelander frames for transparency and generates frames.json.

For each frame:
- `show`: true if the frame has transparent pixels (where user image goes behind)
- `bbox`: [x, y, w, h] bounding box of the transparent region (null if no transparency)
- `file`: the filename
"""

import json
import os
from pathlib import Path
from PIL import Image

FRAMES_DIR = Path(__file__).parent / "frames"
OUTPUT = Path(__file__).parent / "frames.json"

def analyze_frame(path: Path) -> dict:
    img = Image.open(path)

    if img.mode != "RGBA":
        return {"file": path.name, "show": False, "bbox": None}

    alpha = img.getchannel("A")
    # Get bounding box of transparent (alpha < 128) pixels
    # We invert: find where alpha IS low (transparent)
    transparent_pixels = alpha.point(lambda p: 255 if p < 128 else 0)
    bbox = transparent_pixels.getbbox()

    if bbox is None:
        return {"file": path.name, "show": False, "bbox": None}

    return {
        "file": path.name,
        "show": True,
        "bbox": list(bbox),  # [x1, y1, x2, y2]
    }


def main():
    frames = sorted(FRAMES_DIR.glob("Homie-*.png"), key=lambda p: p.name)
    print(f"Found {len(frames)} frames")

    results = []
    transparent_count = 0
    for i, frame in enumerate(frames):
        result = analyze_frame(frame)
        results.append(result)
        if result["show"]:
            transparent_count += 1
        if (i + 1) % 100 == 0:
            print(f"  Processed {i + 1}/{len(frames)}...")

    print(f"\nDone! {transparent_count}/{len(frames)} frames have transparency.")

    with open(OUTPUT, "w") as f:
        json.dump(results, f, separators=(",", ":"))

    print(f"Written to {OUTPUT}")


if __name__ == "__main__":
    main()
