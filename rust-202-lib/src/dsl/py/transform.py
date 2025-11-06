# Example data transformation using Python

def transform(items):
    """Transform a list of numbers"""
    return [x * x for x in items if x % 2 == 0]

def filter_and_map(items, threshold):
    """Filter and map with threshold"""
    return [x * 2 for x in items if x > threshold]

def main():
    """Example main function"""
    test_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    result = transform(test_data)
    return {
        "input": test_data,
        "output": result,
        "count": len(result)
    }

