extern crate arkecosystem_client;

use test_helper::{mock_http_server, mock_client_one, mock_assert_success};

#[test]
fn test_all() {
    let _mock = mock_http_server("blocks");
    {
        let client = mock_client_one();
        let response = client.blocks.all(vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_server("blocks/get");
    {
        let client = mock_client_one();
        let response = client.blocks.show("dummy".to_owned());
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_epoch() {
    let _mock = mock_http_server("blocks/getEpoch");
    {
        let client = mock_client_one();
        let response = client.blocks.epoch();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_height() {
    let _mock = mock_http_server("blocks/getHeight");
    {
        let client = mock_client_one();
        let response = client.blocks.height();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_nethash() {
    let _mock = mock_http_server("blocks/getNethash");
    {
        let client = mock_client_one();
        let response = client.blocks.nethash();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_fee() {
    let _mock = mock_http_server("blocks/getFee");
    {
        let client = mock_client_one();
        let response = client.blocks.fee();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_fees() {
    let _mock = mock_http_server("blocks/getFees");
    {
        let client = mock_client_one();
        let response = client.blocks.fees();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_milestone() {
    let _mock = mock_http_server("blocks/getMilestone");
    {
        let client = mock_client_one();
        let response = client.blocks.milestone();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_reward() {
    let _mock = mock_http_server("blocks/getReward");
    {
        let client = mock_client_one();
        let response = client.blocks.reward();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_supply() {
    let _mock = mock_http_server("blocks/getSupply");
    {
        let client = mock_client_one();
        let response = client.blocks.supply();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_status() {
    let _mock = mock_http_server("blocks/getStatus");
    {
        let client = mock_client_one();
        let response = client.blocks.status();
        mock_assert_success(&_mock, response);
    }
}
