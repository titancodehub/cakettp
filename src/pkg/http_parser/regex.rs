pub const METHOD_REGEX: &str = r"^(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE|LOCK|UNLOCK|PROPFIND|PROPPATCH|COPY|MOVE|MKCOL|MKCALENDAR|ACL|SEARCH)\s+";
pub const COMMENT_REGEX: &str = r"\n#(\s.+?)\\n"; // Example: # This is example format