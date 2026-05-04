import sys
data = {}
for line in sys.stdin:
    if '=' in line:
        k, v = line.strip().split('=', 1)
        data[k] = v
print(f"Hello from Python plugin '{data.get('COMMAND', 'unknown')}'!")
print(f"Running as user: {data.get('USER', 'unknown')}")
print(f"Current directory: {data.get('CWD', '/')}")
print("Plugin executed successfully!")