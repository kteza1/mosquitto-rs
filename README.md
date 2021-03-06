#mosquitto-rs [![travis](https://travis-ci.org/kteza1/mosquitto-rs.svg?branch=master)](https://travis-ci.org/kteza1/mosquitto-rs)

#### DEPRECATED: New pure rust client in development [here](https://github.com/kteza1/rumqtt)

This crate provides rustful wrappers for (unsafe) mosquitto mqtt library.
With these wrappers you can write safe, superfast, concurrent mqtt code.
Since mosquitto libraries are low level and avalilable on almost all the platforms, this crate is super portable
  
[API DOCUMENTATION](http://kteza1.github.io/mosquitto-rs/rustdoc/mosquitto/)


###HOW TO BUILD
---

#####UBUNTU
* sudo apt-add-repository ppa:mosquitto-dev/mosquitto-ppa
* sudo apt-get update
* sudo apt-get install mosquitto
* cargo build


#####MAC OSX
* brew install mosquitto
* cargo build


###SETUP TLS CONNECTIONS

* Generate ca, server, client certificates using the guide [here](http://rockingdlabs.dunmire.org/exercises-experiments/ssl-client-certs-to-secure-mqtt)

* Use the below commands to verify your connection
```
sudo openssl s_client -connect localhost:8884 -CAfile ./ca.crt -cert client.crt -key client.key
```
```
mosquitto_sub -t "ather/log-ship" -v --cafile ca.crt --cert client.crt --key client.key -p 8884
```


####EXTENDING THE MOSQUITTO BROKER FOR 100 THOUSAND CONNECTIONS
---

check [this](https://lists.launchpad.net/mosquitto-users/msg00163.html)
