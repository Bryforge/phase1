import sys

data = {}
for line in sys.stdin:
    if "=" in line:
        key, value = line.strip().split("=", 1)
        data[key] = value

print(f"Hello from Python plugin '{data.get('COMMAND', 'unknown')}'!")
print(f"Running as user: {data.get('USER', 'unknown')}")
print(f"Current directory: {data.get('CWD', '/')}")
print("Plugin executed successfully!")
