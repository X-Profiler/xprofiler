            // 5. HTTP (Placeholder)
            if config.log_format_alinode {
                info("http", "live_http_request: 0, http_request_handled: 0, http_response_sent: 0, http_rt: 0.00");
            } else {
                info("http", &format!("live_http_request: 0, http_response_close: 0, http_response_sent: 0, http_request_timeout: 0, http_patch_timeout: {}, http_rt: 0.00, res: 0", config.patch_http_timeout));
            }