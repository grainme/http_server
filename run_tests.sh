#!/bin/bash
# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'

# Print banner
echo -e "
${BLUE}
  ▄████  ██▀███   ▄▄▄       ██▓ ███▄    █  ███▄ ▄███▓▓█████ 
 ██▒ ▀█▒▓██ ▒ ██▒▒████▄    ▓██▒ ██ ▀█   █ ▓██▒▀█▀ ██▒▓█   ▀ 
▒██░▄▄▄░▓██ ░▄█ ▒▒██  ▀█▄  ▒██▒▓██  ▀█ ██▒▓██    ▓██░▒███   
░▓█  ██▓▒██▀▀█▄  ░██▄▄▄▄██ ░██░▓██▒  ▐▌██▒▒██    ▒██ ▒▓█  ▄ 
░▒▓███▀▒░██▓ ▒██▒ ▓█   ▓██▒░██░▒██░   ▓██░▒██▒   ░██▒░▒████▒
 ░▒   ▒ ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░▓  ░ ▒░   ▒ ▒ ░ ▒░   ░  ░░░ ▒░ ░
  ░   ░   ░▒ ░ ▒░  ▒   ▒▒ ░ ▒ ░░ ░░   ░ ▒░░  ░      ░ ░ ░  ░
░ ░   ░   ░░   ░   ░   ▒    ▒ ░   ░   ░ ░ ░      ░      ░   
      ░    ░           ░  ░ ░           ░        ░      ░  ░${NC}"
echo -e "${YELLOW}⚡ Test Runner v1.0${NC}\n"

# Function to run a specific test
run_test() {
    local test=$1
    echo -e "🚀 ${BLUE}Running test: ${test}${NC}"
    echo -e "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    OUTPUT=$(cargo test --quiet $test -- --exact -- --nocapture 2>&1)
    STATUS=$?

    FILTERED_OUTPUT=$(echo "$OUTPUT" | grep -v "^   Compiling\|^    Finished\|^     Running\|^     Fresh")
    
    if echo "$FILTERED_OUTPUT" | grep -q "test result: FAILED\|panicked"; then
        echo -e "${RED}💥 Test failed${NC}"
        echo -e "${RED}Error:${NC}"
        echo "$FILTERED_OUTPUT" | grep -A 1 "thread.*panicked\|test.*FAILED"
        return 1
    else
        echo -e "${GREEN}✨ Test passed${NC}\n"
        return 0
    fi
}

# Get all available tests in sorted order
get_stage_tests() {
    local stage=$1
    local pattern=$2
    local tests=$(cargo test -- --list 2>&1 | grep ": test$" | sed 's/: test$//' | grep "$pattern")
    if [ ! -z "$tests" ]; then
        echo -e "\n${PURPLE}Stage $stage${NC}"
        echo "$tests"
    fi
}

# Main menu loop
while true; do
    clear
    echo -e "${CYAN}Available tests:${NC}\n"
    
    # Initialize counter
    counter=1
    declare -A test_numbers

    # Stage 1: Bind to port
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "1: Bind to port" "server_")

    # Stage 2: Respond with 200
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "2: Respond with 200" "basic_200\|response_format")

    # Stage 3: Extract URL path
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "3: Extract URL path" "root_path\|simple_path\|nested_path\|path_with")

    # Stage 4: Response with body
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "4: Response with body" "response_with_body\|content_length")

    # Stage 5: Read header
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "5: Read header" "custom_header")

    # Stage 6: Concurrent connections
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "6: Concurrent connections" "concurrent")

    # Stage 7: Read request body
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(get_stage_tests "7: Read request body" "request_body")

    # Error handling and other tests
    echo -e "\n${PURPLE}Error Handling Tests${NC}"
    while IFS= read -r test; do
        if [ ! -z "$test" ]; then
            echo -e "$counter) ${YELLOW}$test${NC}"
            test_numbers[$counter]=$test
            ((counter++))
        fi
    done < <(cargo test -- --list 2>&1 | grep ": test$" | sed 's/: test$//' | \
             grep -v "server_\|basic_200\|response_format\|root_path\|simple_path\|nested_path\|path_with\|response_with_body\|content_length\|custom_header\|concurrent\|request_body")
    
    echo -e "\n${GREEN}a) Run all tests${NC}"
    echo -e "${RED}q) Exit${NC}\n"
    
    read -p "Enter test number or 'a' for all tests (q to quit): " choice
    
    case $choice in
        [0-9]*)
            if [ -n "${test_numbers[$choice]}" ]; then
                test="${test_numbers[$choice]}"
                run_test "$test"
                read -p "Press Enter to continue..."
            else
                echo -e "${RED}Invalid test number${NC}"
                sleep 1
            fi
            ;;
        "a"|"A")
            echo -e "\n${BLUE}Running all tests...${NC}\n"
            passed=0
            total=$((counter-1))
            
            for i in $(seq 1 $total); do
                if ! run_test "${test_numbers[$i]}"; then
                    break
                fi
                ((passed++))
            done
            
            echo -e "\n${BLUE}📊 Test Summary${NC}"
            echo -e "${GREEN}✓ Passed: ${passed}/${total}${NC}"
            if [ $passed -ne $total ]; then
                echo -e "${RED}✗ Some tests failed${NC}"
            fi
            read -p "Press Enter to continue..."
            ;;
        "q"|"Q")
            exit 0
            ;;
        *)
            echo -e "${RED}Invalid choice${NC}"
            sleep 1
            ;;
    esac
done
