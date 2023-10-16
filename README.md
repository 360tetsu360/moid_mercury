# Mercury MOID

This program searches for candidates for the parent bodies of meteor showers that cause Micrometeoroid Impact Vaporization (MMIV), which is one of the causes of Mercury's sodium atmosphere.

It simply searches for objects that are close to the Minimum Orbit Intersection Distance (MOID) of the comet and Mercury, as listed in the [MPC](https://minorplanetcenter.net/iau/Ephemerides/Comets/Soft03Cmt.txt).

The MOID calculation program is based on an algorithm devised by Hans Rickman and Tomasz Wiśniowski. The paper describing this algorithm is T.Wiśniowski and H.Rickman, "A Fast, Geometric Method for Calculating Accurate Minimum Orbit Intersection Distances (MOIDs)," 2013 Acta Astronomica.

Available at: http://moid.cbk.waw.pl/orbity/default/index

In addition, we refer to the code written by Mike Kretlow to modify this program into a function and call it in Rust.

https://github.com/mkretlow/MOID.jl/tree/master