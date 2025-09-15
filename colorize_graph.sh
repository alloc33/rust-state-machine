#!/bin/bash
sed -e 's/\[label="pub trait[^"]*"/& fillcolor=lightblue style=filled/g' \
    -e 's/\[label="pub struct[^"]*"/& fillcolor=lightgreen style=filled/g' \
    -e 's/\[label="pub enum[^"]*"/& fillcolor=lightyellow style=filled/g' \
    -e 's/\[label="pub type[^"]*"/& fillcolor=lightcoral style=filled/g' \
    -e 's/\[label="pub.*mod[^"]*"/& fillcolor=lightgray style=filled/g'
