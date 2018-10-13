env -i TKEY=VAL ruby -r pp -r time <<__EOS__
puts "PID: #{\$\$}"
print "Startup environment: "
pp ENV

loop {
  ENV['DYNAMIC_KEY'] = Time.now.to_s
  print "Current environment: "
  pp ENV
  sleep 5
}
__EOS__
