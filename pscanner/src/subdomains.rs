use http_client::HttpClient;

pub fn enumerate(http_client: &dyn HttpClient, target: &str) -> Result<Vec<Subdomain>, Error> {
    //  ...
}