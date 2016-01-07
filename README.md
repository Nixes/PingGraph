# PingGraph
Simple rust utility to graph ping over time.


![Alt text](ping_graph_screenshot.jpg)

Just wraps windows ping utility and uses piston with OpenGL backend for rendering. Only the ping command parsing is windows specific, everything else is platform independent.

Pings google.com by default, which should always redirect to localised version and thus give values between 30 and 100ms for most users.
