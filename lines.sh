#!/usr/bin/env bash

# Colors and icons
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

CHART=""
FILE=""
WARN=""

echo -e "${BLUE}${CHART}  Line Count Analysis${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

# Find all .rs files
rs_files=$(find . -name "*.rs" -not -path "./target/*" | sort)

total_lines=0
total_files=0
max_lines=0
max_file=""
min_lines=999999
min_file=""

echo -e "${BLUE}${FILE}  File Analysis:${NC}"
echo ""

# Analyze each file
while IFS= read -r file; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        total_lines=$((total_lines + lines))
        total_files=$((total_files + 1))

        # Track max
        if [ $lines -gt $max_lines ]; then
            max_lines=$lines
            max_file=$file
        fi

        # Track min
        if [ $lines -lt $min_lines ]; then
            min_lines=$lines
            min_file=$file
        fi

        # Color code by size
        if [ $lines -gt 100 ]; then
            color=$RED
            icon="${WARN}"
        elif [ $lines -gt 80 ]; then
            color=$YELLOW
            icon="${WARN}"
        else
            color=$GREEN
            icon=" "
        fi

        printf "${color}${icon}  %4d lines${NC}  %s\n" $lines "$file"
    fi
done <<< "$rs_files"

# Calculate average
if [ $total_files -gt 0 ]; then
    avg_lines=$((total_lines / total_files))
else
    avg_lines=0
fi

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Summary:${NC}"
echo -e "  Total files:   ${total_files}"
echo -e "  Total lines:   ${total_lines}"
echo -e "  Average lines: ${avg_lines}"
echo -e "  Max lines:     ${max_lines} ${YELLOW}(${max_file})${NC}"
echo -e "  Min lines:     ${min_lines} ${GREEN}(${min_file})${NC}"
echo ""

# Check if we have files over 100 lines
over_100=$(find . -name "*.rs" -not -path "./target/*" -exec wc -l {} \; | awk '$1 > 100 {count++} END {print count+0}')

if [ $over_100 -gt 0 ]; then
    echo -e "${YELLOW}${WARN}  Warning: ${over_100} file(s) exceed 100 lines${NC}"
else
    echo -e "${GREEN}  All files under 100 lines!${NC}"
fi
