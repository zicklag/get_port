# get-port
> Get an available port

---

## Usage

```rust
use get_port;

fn main() {
    let an_available_port = get_port::get_port().unwrap(); // Returns the first available port in default range
    let an_available_port_in_range = get_port::get_port_in_range(get_port::PortRange { min: 5000, max: 6000 }).unwrap(); // Returns the first available port in speciefied range

    // ...
}
```

---

## Future
* [x] Add `get_port_prefer`, used to supply (a list of) prefered port(s)
    - falls back to `get_port` (which runs a boundless port availability check through PortRange::default())
        - * [ ] Add `get_port_prefer_or_range`
* [ ] Add `get_port_except` to exclude certain port(s)
* [ ] Write meaningful tests?
