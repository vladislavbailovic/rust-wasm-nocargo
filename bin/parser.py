#!/usr/bin/env python

from xml.dom.minidom import parse


def run():
    dom = parse("/home/ve/Downloads/map.osm")
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
    out.append(f"static MAP_NODES: [Node; {len(nodes)}] = [")
    for node_id in nodes:
        node = nodes[node_id]
        out.append("\tNode {")
        out.append(f"\t\tlat: {node['lat']},")
        out.append(f"\t\tlon: {node['lon']},")
        out.append("\t},")
    out.append("];")
    return "\n".join(out)

if __name__ == "__main__":
    run()
