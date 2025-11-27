// This function gets an env variable and throws if it's not set
export function env(key: string): string {
  const value = process.env[key];
  if (!value) throw new Error(`Environment variable ${key} is not set but is required.`);
  return value;
}

// This function gets a comma-separated env variable and returns an array
export function envArray(key: string): string[] {
  return env(key).split(',').map(item => item.trim());
}
