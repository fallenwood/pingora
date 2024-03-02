// Copyright 2024 Cloudflare, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use pingora::server::Server;

mod app;
mod service;

pub fn main() {
    env_logger::init();

    let conf = pingora::server::configuration::ServerConf::from_yaml("---\nversion: 1\nthreads: 8").ok().unwrap();

    let mut my_server = Server::new_with_conf(None, conf).unwrap();
    my_server.bootstrap();

    // should be 8, default is 1
    println!("Threads: {}", my_server.configuration.threads);

    let mut echo_service_http = service::echo::echo_service_http();
    echo_service_http.add_tcp("0.0.0.0:6145");
    echo_service_http.add_uds("/tmp/echo.sock", None);

    my_server.add_service(echo_service_http);
    my_server.run_forever();
}
