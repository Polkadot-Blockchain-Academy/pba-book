#!/bin/bash
# Scan for images:
# - Files present, but not used as `<img src="{PATH TO IMAGE FILE}" />` divs (img required, per convention)
# - `<img src="{PATH TO IMAGE FILE}" />` elements defined, but `src="..."` is not found in files

if [ -z $1 ] 
then 
    echo -e "This script expects you provide a path to recusively search in for all files (slides, book pages, img, etc.)\nExample:\n"
    echo -e "./orphan-image-scan.sh ../../content/\n"
    exit 1
else
    contentpath="$1"
fi

echo "📸 Scanning for image issues in: $contentpath"


# RAM only temp dir
# https://unix.stackexchange.com/a/301817
ramdir=$(mktemp -dt "$(basename $0).XXXXXXXX" --tmpdir=/run/user/$(id -u))

# Find all <img /> sources
grep -hrPo '<img.*src="\K.*?(?=".*)' $contentpath > $ramdir/img-element-list
awk -F ":" '{print $NF}' $ramdir/img-element-list | awk -F "/" '{print $NF}' | sort | uniq > $ramdir/img-element-files

# Find all img flies
find $contentpath \
-name "*.png" \
-o -name "*.svg" \
-o -name "*.jpg" \
-o -name "*.jpeg" \
-o -name "*.webp" \
-o -name "*.gif" \
-o -name "*.avif" \
> $ramdir/img-asset-path
awk -F "/" '{print $NF}' $ramdir/img-asset-path | sort | uniq > $ramdir/img-asset-files

# Check where files and <img src="..."> are not congruent with assets included
comm -13 $ramdir/img-element-files $ramdir/img-asset-files > $ramdir/img-to-delete
comm -13 $ramdir/img-asset-files $ramdir/img-element-files > $ramdir/img-no-src-asset

# DEBUG view the tmp files with important info
# TODO - expand this script if needed to automatically prune/fix issues
if [ -s $ramdir/img-no-src-asset ]; then
    echo -e "🤷 Images used in book, but not found in files (should be saved in book assets, in right path):\n"
    cat $ramdir/img-no-src-asset
    echo -e "\n➤ DEBUG - above list saved to: $ramdir/img-no-src-asset --- more debugging files in: $ramdir/"
fi
if [ -s $ramdir/img-to-delete ]; then
    echo -e "❌ Image files included, but not used in book (delete if not needed):\n"
    cat $ramdir/img-to-delete
    echo -e "\n➤ DEBUG - above list saved to: $ramdir/img-to-delete --- more debugging files in: $ramdir/"
fi
if [ -s $ramdir/img-to-delete ] || [ -s $ramdir/img-no-src-asset ]; then
    exit 1
fi

# Don't clean up if needed to debug 😜
echo "✅ No issues with images 🎉"
rm -rf $ramdir
exit 0
