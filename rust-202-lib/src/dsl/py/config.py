# Example configuration file using Python as DSL

DATABASE_URL = "postgresql://localhost:5432/rust202"
REDIS_URL = "redis://localhost:6379"
MAX_CONNECTIONS = 100
TIMEOUT_SECONDS = 30

FEATURE_FLAGS = {
    "enable_caching": True,
    "enable_metrics": True,
    "debug_mode": False,
}

def get_config():
    """Return configuration as dict"""
    return {
        "database_url": DATABASE_URL,
        "redis_url": REDIS_URL,
        "max_connections": MAX_CONNECTIONS,
        "timeout": TIMEOUT_SECONDS,
        "features": FEATURE_FLAGS,
    }

def main():
    return get_config()

