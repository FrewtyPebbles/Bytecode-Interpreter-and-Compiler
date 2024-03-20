import time

start = time.time_ns()
for i in range(0,1000):
    print("Hello!")
    print("My name is William.")
end = time.time_ns()
print(f"runtime: {(end-start)/1_000_000}ms")