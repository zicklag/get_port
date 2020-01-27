# get-port
> Get an available port

---

## Usage

#### * Get the first available port in range 1024 <-> 65535
```rust
use get_port;

fn main() {
    let port = get_port::get_port().unwrap();
}
```

#### * Get a port in a specific range

```rust
use get_port;

fn main() {
    let port = get_port::get_port_in_range(get_port::PortRange { min: 5000, max: 6000 }).unwrap();
}
```

#### * Check if specific port(s) is/are available, returning the first one or falling back to an available port in range 1024 <-> 65535

```rust
use get_port;

fn main() {
    let port = get_port::get_port_prefer(vec![20, 60, 6943]).unwrap(); // Will return 6943 if available, as 0 <-> 1024 are system ports.
}
```
---

## Future
* [x] Add `get_port_prefer`, used to supply (a list of) prefered port(s)
    - falls back to `get_port` (which runs a boundless port availability check through PortRange::default())
        - * [ ] Add `get_port_prefer_or_range`
* [ ] Add `get_port_except` to exclude certain port(s)
* [ ] Write meaningful tests?
