# 0.2.3
- Refactored the detection of which methods exist,
  we actually parse the file now instead of just checking that it contains `pub async #method_name`

# 0.2.2
- Re-licensed to MIT

# 0.2.1
- Documentation & test improvements

# 0.2.0
- Generate module imports instead of `include!`ing, so rust-analyzer works.

# 0.1.0
- MVP adapted from https://github.com/richardanaya/axum-folder-router-htmx
