# 使用官方的Rust基础镜像作为构建环境
FROM rust:1.67 as builder
WORKDIR /builder

# 复制项目和Cargo.toml
COPY . .

# 使用cargo构建项目
RUN cargo install --path .

# 使用Distroless作为运行时环境
FROM gcr.io/distroless/cc-debian10
COPY --from=builder /builder/target/release/simple_rust_proxy /usr/local/bin/simple_rust_proxy
CMD ["simple_rust_proxy"]