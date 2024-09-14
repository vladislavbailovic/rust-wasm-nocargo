#!/usr/bin/env python

import sys
from xml.dom.minidom import parse


def run(filename):
    dom = parse(filename)
    latlon = {}
    for node in dom.getElementsByTagName("node"):
        node_id = node.getAttribute("lon")
        if not node_id:
            continue
        node = dict(
            lat=node.getAttribute("lat"),
            lon=node.getAttribute("lon"),
        )
        latlon[node_id] = node
    rust = printRust(latlon)
    print(rust)

def printRust(nodes):
    out = ["use Node;"]
    out.append(f"pub static MAP_NODES: [Node; {len(nodes)}] = [")
    for node_id in nodes:
        node = nodes[node_id]
        out.append("\tNode {")
        out.append(f"\t\tlat: {node['lat']},")
        out.append(f"\t\tlon: {node['lon']},")
        out.append("\t},")
    out.append("];")
    return "\n".join(out)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage {sys.argv[0]} <map_data.osm>")
    else:
        run(sys.argv[1])
