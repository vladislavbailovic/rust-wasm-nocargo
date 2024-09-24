#!/usr/bin/env python

import sys
from collections import OrderedDict
from xml.dom.minidom import parse


def run(filename):
    dom = parse(filename)
    nodes = OrderedDict()
    for node in dom.getElementsByTagName("node"):
        node_id = node.getAttribute("id")
        if not node_id:
            continue
        # print(f"node: {node.getAttribute('lat')}x{node.getAttribute('lon')}")
        # for tag in node.getElementsByTagName("tag"):
        #     print(f"\t{tag.getAttribute('k')}: {tag.getAttribute('v')}")
        node = dict(
            lat=node.getAttribute("lat"),
            lon=node.getAttribute("lon"),
            idx=len(nodes),
        )
        nodes[node_id] = node

    ways = []
    for wt in dom.getElementsByTagName("way"):
        way = {
            "nodes": [],
        }
        for nd in wt.getElementsByTagName("nd"):
            ref = nd.getAttribute("ref")
            if not ref in nodes:
                raise Error("missing node")
            node = nodes.get(ref)
            way["nodes"].append(node["idx"])
        if len(way["nodes"]) < 2:
            continue
        ways.append(way)
    rust = printRust(nodes, ways)
    print(rust)

def printRust(nodes, ways):
    out = [
        "use std::sync::Once;",
        "use Node;",
    ]
    out.append(printNodes(nodes))
    out.append(printWays(ways))
    return "\n".join(out)

def printWays(ways):
    out = [
        "static mut MAP_WAYS: Vec<Vec<usize>> = Vec::new();",
        "static WAYS_LOCK: Once = Once::new();",
        "pub fn get_map_ways() -> &'static [Vec<usize>] {",
        "    unsafe {",
        "        WAYS_LOCK.call_once(|| {",
        "            MAP_WAYS = vec![",
    ]
    for way in ways:
        nodes = ",".join(map(str, way["nodes"]))
        out.append(f"\t\t\t\tvec![{nodes}],")
    out.extend([
        "            ];",
        "        });",
        "        &MAP_WAYS",
        "    }",
        "}",
    ])
    return "\n".join(out)

def printNodes(nodes):
    out = [f"pub static MAP_NODES: [Node; {len(nodes)}] = ["]
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
