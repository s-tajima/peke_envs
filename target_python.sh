env -i TKEY=VAL python <<__EOS__
from time import sleep
import os
import datetime

print("PID: %d" % os.getpid())
print("Starting environment: %s" % os.environ)

while True:
    os.environ["DYNAMIC_KEY"] = datetime.datetime.today().strftime('%Y-%m-%d %H:%M:%S')
    print("Current environment : %s" % os.environ)
    sleep(5)
__EOS__
