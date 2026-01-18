# Study Time Tracker – Rust Smart Contract

A beginner-friendly Rust smart contract that helps students track their study hours securely.

## What it does
- Tracks total study hours for a student
- Only the owner (student) can update or reset hours
- Prevents unauthorized access

## Design choices
- Uses ownership-based access control
- Simple state structure for clarity
- Explicit error handling using Result

## State & flow
- Contract initializes with 0 hours
- `log_hour` increments study time
- `reset` clears hours (owner only)
- `get_hours` returns current value

## Security considerations
- Caller verification before state mutation
- Prevents unauthorized updates

## Possible improvements
- Support multiple students
- Add timestamps for sessions
- Role-based access
