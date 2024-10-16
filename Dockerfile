FROM scratch
COPY  target/x86_64-unknown-linux-musl/debug/children_of_pid_1 /init
CMD ["/init"]