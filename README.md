# Hyperb

Experimental ruby gem wrapper around a rust crate using [Helix](https://github.com/tildeio/helix).

You shouldn't use this for anything because it has very little functionality.

_Technically_ this uses the [reqwest](https://github.com/seanmonstar/reqwest) crate, which is a high level wrapper of [hyper](https://github.com/hyperium/hyper) with some nice-ities like tls built in, but Hyperb is a cooler name than rbwequest or reqwestrb.

##### Playing around:
 _because let's be realistic, you shouldn't actually use this in any project you want to function correctly ;)_

 Clone the repo, run `rake`. It'll build and run the spec suite.

 If you'd like to make some GET requests, `bin/console` and `Hyperb.get("www.example.com")`.

 Right now you'll get a hash similar to:
 ```ruby
 {
    body: "response body",
    headers: { "key" => "value" },
    status:  "200 OK"
 }
 ```
