#![feature(custom_derive)]

extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Headers, UserAgent};

extern crate serde_json;
use serde_json::Value;

// Github API
//    https://developer.github.com/v3

// See github api to search repositories:
//     https://developer.github.com/v3/search/#search-repositories
// url:
//     https://api.github.com/search/repositories?q=iron+language:rust&sort=stars&order=desc


// api versioning
// --------------
// explicitly request this version via the Accept header.
// Accept: application/vnd.github.v3+json


// Rate Limiting
// --------------
//    https://developer.github.com/v3/#rate-limiting
// For requests using Basic Authentication or OAuth, you can make up to 5,000 requests per hour.
// For unauthenticated requests, the rate limit allows you to make up to 60 requests per hour.
// $ curl -i https://api.github.com/users/whatever
//     HTTP/1.1 200 OK
//     Date: Mon, 01 Jul 2013 17:27:06 GMT
//     Status: 200 OK
//     X-RateLimit-Limit: 60
//     X-RateLimit-Remaining: 56
//     X-RateLimit-Reset: 1372700873

// Header Name     Description
// X-RateLimit-Limit   The maximum number of requests that the consumer is permitted to make per hour.
// X-RateLimit-Remaining   The number of requests remaining in the current rate limit window.
// X-RateLimit-Reset   The time at which the current rate limit window resets in UTC epoch seconds.

// https://github.com/tj/node-ratelimiter


// token
// -----

// generate/regenerate access token
//   https://github.com/settings/tokens
//

// User-Agent
// ----------
// All API requests MUST include a valid User-Agent header. Requests with no User-Agent header will be rejected.

// Pagination
// ----------

// expose headers
// --------------
// Access-Control-Expose-Headers: ETag, Link, X-GitHub-OTP, X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, X-OAuth-Scopes, X-Accepted-OAuth-Scopes, X-Poll-Interval

// cors
// ----
// Access-Control-Allow-Origin: *


// gists
// -----
// types: Public gists, Secret gists, Anonymous gists
// https://help.github.com/articles/about-gists/
// http://techslides.com/github-gist-api-with-curl-and-ajax
// https://github.com/defunkt/gist
// in rust playgroung, all gists are anonymous gists
//
// curl -H "Content-Type: application/json" -d '{"description": "the description for this gist","public": true,"files": {"file1.txt": {"content": "anonymous gist"}}}' https://api.github.com/gists
//   new anonymous gist:  https://api.github.com/gists/7fb282a4d47b16973a2db7190315c552
// curl -s https://api.github.com/gists/7fb282a4d47b16973a2db7190315c552

// Truncation
//   The Gist API provides up to one megabyte of content for each file in the gist.
//   Every call to retrieve a gist through the API has a key called truncated. If truncated is true,
//   the file is too large and only a portion of the contents were returned in content.
//   If you need the full contents of the file, you can make a GET request to the URL specified by raw_url.
//   Be aware that for files larger than ten megabytes, you’ll need to clone the gist via the URL provided by git_pull_url.


// Github Api security internal
//
// # _fi_sess is must have. Still do not know how to get this id.
// # authenticity_token is must have.???
// #  <!-- </textarea> --><!-- '"` --><meta content="authenticity_token" name="csrf-param" />
// #  <meta content="alHFJODwutAVXd+wtR/LVqHPLZbXMZSYsesGDL2uhq/sQQDmsSWK9UyCbFL+EcYCpDlqfg9ChpEtt1K5UTwOWA==" name="csrf-token" />
// #  aFdKQG9EsHAcrJXSqCxqLK0PdO8LxfwvIvRVte4wpzRv8/e4pxYAW9iC91j5gQI8wmMJm0xyYnY4XLv/8y0bQQ==
// curl 'https://git.autodesk.com/session'    -H 'Cookie: _fi_sess=eyJzZXNzaW9uX2lkIjoiMzcxMzA1MjNhMmI3ODZlMGFjMzAxZDJhMWZiODQ4OTAiLCJfY3NyZl90b2tlbiI6Im1GWW8vanVoV0JoQjdWVUJaYWNJcUxpZW1wQmF0NVoyS0lCaGVJeVJBZ1U9In0%3D--47e0bf50009dd3e74fd68fd5fd3c5ccd583fece7;   tz=Asia%2FShanghai' -H 'Connection: keep-alive' -H 'DNT: 1' --data 'utf8=%E2%9C%93&authenticity_token=PTOZ0K%2Ft6FtYmG6%2F%2B3wkGP1uGs7sokSWvizPwWNBpI8d1a2k0DfHx0TbrmGIClnH26nLwQpyVbadLr4VO0Uu%2FQ%3D%3D&login=gongp&password=ADSKwangbadan4&return_to=https%3A%2F%2Fgit.autodesk.com%2FHBP%2FRAAS' --compressed --include
//
// # user_session is must have
// curl 'https://git.autodesk.com/HBP/RAAS'   -H 'Referer: https://git.autodesk.com/' -H 'Cookie:  user_session=r6FgcO3g-E5qivzqxFk3bNLc_vdAJk-lLXxDrGl22vIWR3_8VwcY_w1YtRjlBfbtJzzv-u_ql6h9nu2F; logged_in=yes; dotcom_user=gongp; '



// some github apps
// https://github.com/askobara/gitostat
// https://github.com/QuentinPerez/gh-repo-contributions
// https://github.com/baya/Gstar
//
fn github_request(url: &String) ->String {

    let mut headers = Headers::new();
    headers.set(UserAgent("playground-rust".to_string()));

    let client = Client::new();
    let mut res = client.get(url)
        .headers(headers)
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}

/// host: host name
/// keyword: keyword
/// ua: user agent
fn search_github_hyper_repos(host: &String, keyword: &String) {

    let url:String = format!("{}/search/repositories?q={}+language:rust&sort=stars&order=desc", host, keyword);

    let body = github_request(&url);

    let data:Value = serde_json::from_str(&body).unwrap();

    let obj = data.as_object().unwrap();
    let total_acount = obj.get("total_count").unwrap();

    let itemsdata = obj.get("items").unwrap();
    let items = itemsdata.as_array().unwrap();

    println!("{:?} repositories by keyword: {}", total_acount, keyword);

    // Rate limitation
    let connetions = if items.len() > 5 { 5 } else { items.len() };
    for i in 0..connetions {

        let item = items.get(i).unwrap();

        let itemobj = item.as_object().unwrap();
        let full_name = itemobj.get("full_name").unwrap();
        let forks_url_data = itemobj.get("forks_url").unwrap();

        let forks_url_string = forks_url_data.as_string().unwrap();
        let forks_body = github_request(&forks_url_string.to_string());

        let forks_data:Value = serde_json::from_str(&forks_body).unwrap();
        let forks = forks_data.as_array().unwrap();

        println!("{:?}: {} forks", full_name, forks.len());
    }
}



// API Host:
//    Github Enterpreise: http(s)://hostname/api/v3/
//    Github : https://api.github.com

fn main() {

    search_github_hyper_repos(
        &("https://api.github.com".to_string()),
        &("hyper".to_string())
    );
}