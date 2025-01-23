import socket
import time
from dataclasses import dataclass
from typing import List, Callable
import threading
import subprocess
import sys
from enum import Enum

class TestResult(Enum):
    PASS = "PASS"
    FAIL = "FAIL"

@dataclass
class TestCase:
    name: str
    description: str
    test_function: Callable
    stage: int

class HTTPServerTester:
    def __init__(self, server_command: List[str], port: int = 4221):
        self.server_command = server_command
        self.port = port
        self.server_process = None
        self.test_cases = []

    def add_test(self, test_case: TestCase):
        self.test_cases.append(test_case)
    
    def _start_server(self):
        self.server_process = subprocess.Popen(
            self.server_command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )
        # Give the server a moment to start
        time.sleep(1)
    
    def _stop_server(self):
        if self.server_process:
            self.server_process.terminate()
            self.server_process.wait()
    
    def _send_request(self, request: str) -> tuple[str, int]:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect(('localhost', self.port))
            s.sendall(request.encode())
            response = s.recv(1024).decode()
            return response
    
    def run_tests(self, stage: int = None) -> List[tuple[TestCase, TestResult, str]]:
        results = []
        
        for test_case in self.test_cases:
            if stage is not None and test_case.stage != stage:
                continue
                
            print(f"\nRunning test: {test_case.name}")
            print(f"Description: {test_case.description}")
            
            try:
                self._start_server()
                success = test_case.test_function(self._send_request)
                results.append((test_case, TestResult.PASS if success else TestResult.FAIL, ""))
            except Exception as e:
                results.append((test_case, TestResult.FAIL, str(e)))
            finally:
                self._stop_server()
        
        return results

# Example test cases for the first 4 stages
def create_basic_test_suite() -> HTTPServerTester:
    tester = HTTPServerTester(["./your_server_binary"])  # User would specify their server command
    
    # Stage 1: Bind to port
    def test_server_starts(send_request):
        try:
            response = send_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            return True
        except ConnectionRefusedError:
            return False
    
    tester.add_test(TestCase(
        "Server Binding",
        "Tests if server successfully binds to port and accepts connections",
        test_server_starts,
        1
    ))
    
    # Stage 2: Respond with 200
    def test_200_response(send_request):
        response = send_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        return "HTTP/1.1 200 OK" in response
    
    tester.add_test(TestCase(
        "200 Response",
        "Tests if server responds with 200 OK to basic GET request",
        test_200_response,
        2
    ))
    
    # Stage 3: Extract URL path
    def test_url_path(send_request):
        # Test root path
        response = send_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        if "HTTP/1.1 200 OK" not in response:
            return False
            
        # Test non-existent path
        response = send_request("GET /not-found HTTP/1.1\r\nHost: localhost\r\n\r\n")
        return "HTTP/1.1 404 Not Found" in response
    
    tester.add_test(TestCase(
        "URL Path Handling",
        "Tests if server correctly handles different URL paths",
        test_url_path,
        3
    ))
    
    return tester

def main():
    tester = create_basic_test_suite()
    results = tester.run_tests()
    
    # Print results
    print("\nTest Results:")
    for test_case, result, error in results:
        print(f"{test_case.name}: {result.value}")
        if error:
            print(f"Error: {error}")

if __name__ == "__main__":
    main()
