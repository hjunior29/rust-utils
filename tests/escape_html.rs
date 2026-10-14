#[cfg(test)]
mod tests {
    use rust_utils::escape_html;

    #[test]
    fn test_escape_html_empty() {
        let input = "";
        let result = escape_html(input);
        assert_eq!(result, "");
    }

    #[test]
    fn test_escape_html_no_special_chars() {
        let input = "Hello world 123!";
        let result = escape_html(input);
        assert_eq!(result, "Hello world 123!");
    }

    #[test]
    fn test_escape_html_all_special_chars() {
        let input = "& < > \" '";
        let result = escape_html(input);
        assert_eq!(result, "&amp; &lt; &gt; &quot; &#x27;");
    }

    #[test]
    fn test_escape_html_mixed_content() {
        let input = "<script>alert('Hello & welcome');</script>";
        let result = escape_html(input);
        assert_eq!(
            result,
            "&lt;script&gt;alert(&#x27;Hello &amp; welcome&#x27;);&lt;/script&gt;"
        );
    }
}
