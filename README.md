# EC2 Termination Handler - written in Rust

This is an EC2 termination handler that detects whether the instance will go down and sends a message 
to its logs so that you know whether an instance has been terminated. 

## Integration Testing

  * [ ] use Handlebars (https://github.com/sunng87/handlebars-rust/blob/master/examples/render_file.rs) to configure a way to configure the test and monitor instances to talk to each other. Test instances have fluent-bit running and they need to push logs to Loki (https://grafana.com/docs/loki/latest/send-data/fluentbit/)

  * [ ] Use octocrab (https://github.com/XAMPPRocky/octocrab) to get the artifact from GH action and download it to the test instance. 
  
  * [ ] Use FIS (https://docs.aws.amazon.com/fis/latest/userguide/fis-tutorial-spot-interruptions.html) to test spot interruptions.
