pub mod genieacs;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_string() {
        let input = "Hello World!";
        let expected = "Hello%20World%21";

        assert_eq!(utils::url::encode(input.to_string()), expected);
    }

    #[tokio::test]
    async fn test_query_device_by_param() {
        let genie = genieacs::GenieACS::new("http://localhost:7557/");
        let query = genie
            .find_device("_id", "0425E0-G%2D240W%2DG-ALCLB3FA4652")
            .await;
    }
}
