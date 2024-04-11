#!/bin/bash

locale="en-US" # Default locale
holidays=()    # Default holidays list

is_workday() {
    local date="$1"
    local dt=$(date -d "$date" +%u)

    # Check if it's a weekend
    if [ $dt -gt 5 ]; then
        return 1
    fi

    # Check if it's a holiday
    for holiday in "${holidays[@]}"; do
        if [ "$date" == "$holiday" ]; then
            return 1
        fi
    done

    return 0
}

update() {
    local new_locale="$1"
    shift
    local new_holidays=("$@")
    locale="$new_locale"
    holidays+=("${new_holidays[@]}")
}

get_next_workday() {
    local date="$1"
    local dt=$(date -d "$date + 1 day" +%Y-%m-%d)

    while ! is_workday "$dt"; do
        dt=$(date -d "$dt + 1 day" +%Y-%m-%d)
    done

    echo "$dt"
}

is_holiday() {
    local date="$1"
    for holiday in "${holidays[@]}"; do
        if [ "$date" == "$holiday" ]; then
            return 0
        fi
    done
    return 1
}

add_holiday() {
    local date="$1"
    holidays+=("$date")
}

remove_holiday() {
    local date="$1"
    local idx
    for ((idx=0; idx<${#holidays[@]}; idx++)); do
        if [ "${holidays[idx]}" == "$date" ]; then
            unset 'holidays[idx]'
            holidays=("${holidays[@]}")
            return
        fi
    done
}

get_workdays_in_range() {
    local start_date="$1"
    local end_date="$2"
    local current_date="$start_date"

    while [ "$current_date" != "$end_date" ]; do
        if is_workday "$current_date"; then
            echo "$current_date"
        fi
        current_date=$(date -d "$current_date + 1 day" +%Y-%m-%d)
    done

    if is_workday "$end_date"; then
        echo "$end_date"
    fi
}

get_holiday_name() {
    local date="$1"
    for holiday in "${holidays[@]}"; do
        if [ "$date" == "$holiday" ]; then
            echo "$holiday"
            return
        fi
    done
    echo "Not a holiday"
}

# Example usage:
# update "en-US" "2024-01-01" "2024-12-25"
# is_workday "2024-04-15"
# get_next_workday "2024-04-15"
# is_holiday "2024-04-15"
# add_holiday "2024-05-01"
# remove_holiday "2024-12-25"
# get_workdays_in_range "2024-04-01" "2024-04-30"
# get_holiday_name "2024-04-15"
