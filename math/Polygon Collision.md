Ray : $R = R_0 + r R_1$  
Polygon : $P = P_0 + p_1 P_1 + p_2 P_2$  
If $R = P$ then we have a collision.  
$$R = P$$
$$R_0 + r R_1 = P_0 + p_1 P_1 + p_2 P_2$$
$$\begin{cases}
R_{0x} + r R_{1x} = P_{0x} + p_1 P_{1x} + p_2 P_{2x} \\
R_{0y} + r R_{1y} = P_{0y} + p_1 P_{1y} + p_2 P_{2y} \\
R_{0z} + r R_{1z} = P_{0z} + p_1 P_{1z} + p_2 P_{2z}
\end{cases}$$
$$R_{0x} + r R_{1x} = P_{0x} + p_1 P_{1x} + p_2 P_{2x}$$
$$r R_{1x} = P_{0x} + p_1 P_{1x} + p_2 P_{2x} - R_{0x}$$
$$r = \frac{P_{0x} + p_1 P_{1x} + p_2 P_{2x} - R_{0x}}{R_{1x}}$$
---
$$R_{0y} + \frac{P_{0x} + p_1 P_{1x} + p_2 P_{2x} - R_{0x}}{R_{1x}} R_{1y} - P_{0y} - p_2 P_{2y} = p_1 P_{1y}$$
$$p_1 = \frac{R_{0y} + \frac{P_{0x} + p_1 P_{1x} + p_2 P_{2x} - R_{0x}}{R_{1x}} R_{1y} - P_{0y} - p_2 P_{2y}}{P_{1y}}$$
$$p_1 = \frac{P_{0x} + p_1 P_{1x} + p_2 P_{2x} - R_{0x}}{R_{1x}P_{1y}} R_{1y} + \frac{R_{0y} - P_{0y} - p_2 P_{2y}}{P_{1y}}$$
$$p_1 = \frac{p_1 P_{1x} R_{1y}}{R_{1x}P_{1y}} + \frac{P_{0x} + p_2 P_{2x} - R_{0x}}{R_{1x}P_{1y}} R_{1y} + \frac{R_{0y} - P_{0y} - p_2 P_{2y}}{P_{1y}}$$
$$p_1 = \frac{P_{0x} + p_2 P_{2x} - R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y} + \frac{R_{0y} - P_{0y} - p_2 P_{2y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}$$
Simplify for $p_2$
$$
p_1
= \frac{p_2 P_{2x} R_{1y}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{P_{0x} - R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
- \frac{p_2 P_{2y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{R_{0y} - P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
$$
$$
p_1
= p_2 \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
$$
---
$$R_{0z}
+ r R_{1z}
= P_{0z}
+ p_1 P_{1z}
+ p_2 P_{2z}$$
$$
R_{0z}
+ \frac{P_{0x}
+ \left( p_2 \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} \right) P_{1x}
+ p_2 P_{2x}
- R_{0x}}{R_{1x}} R_{1z}
= P_{0z}
+ \left(\frac{P_{0x}
+ p_2 P_{2x}
- R_{0x}}
{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}
- p_2 P_{2y}}
{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}\right) P_{1z}
+ p_2 P_{2z}
$$
$$
R_{0z}
+ \frac{P_{0x}
+ \left( p_2 \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} \right) P_{1x}
+ p_2 P_{2x}
- R_{0x}}{R_{1x}} R_{1z}
= P_{0z}
+ \left(
p_2 \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
\right) P_{1z}
+ p_2 P_{2z}
$$
$$
\frac{p_2 \left( P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}} \right) P_{1x}}{R_{1x} R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
R_{0z}
+ \frac{p_2 P_{2x}}{R_{1x}}
+ \frac{P_{0x}
+ \left( \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} \right) P_{1x}
- R_{0x}}{R_{1x}} R_{1z}
= P_{0z}
+ p_2 P_{1z} \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
+ \left( \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
\right) P_{1z}
+ p_2 P_{2z}
$$
$$
p_2
= \frac{ P_{0z}
+ \left( \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
\right) P_{1z}
- \frac{P_{0x}
+ \left( \frac{P_{0x}
- R_{0x}}{R_{1x}P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} R_{1y}
+ \frac{R_{0y}
- P_{0y}}{P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)} \right) P_{1x}
- R_{0x}}{R_{1x}} R_{1z}}
{\frac{ \left( P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}} \right) P_{1x}}{R_{1x} R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
R_{0z}
+ \frac{P_{2x}}{R_{1x}}
- P_{1z} \frac{P_{2x} R_{1y}
- \frac{P_{2y}}{R_{1x}}}{R_{1x} P_{1y}\left( 1 - \frac{P_{1x} R_{1y}}{R_{1x}P_{1y}}\right)}
- P_{2z}}
$$
$$
p_2
= \frac{ P_{0z}
+ \left( \frac{R_{1y}P_{1z}P_{0x}
- R_{1y}P_{1z}R_{0x}
+ R_{1x} P_{1z} R_{0y}
- R_{1x} P_{1z} P_{0y}}{R_{1x} P_{1y} - P_{1x} R_{1y}}
\right)
- \frac{R_{1z} P_{0x}}{R_{1x}}
+ \left( \frac{R_{1z} P_{1x}}{R_{1x}} \frac{R_{1y} P_{0x}
- R_{1y} R_{0x}}{R_{1x} P_{1y} - P_{1x} R_{1y}}
+ \frac{R_{1z} P_{1x}}{R_{1x}} \frac{R_{1x} R_{0y}
- R_{1x} P_{0y}}{P_{1y} R_{1x} - P_{1x} R_{1y}} \right)
- \frac{R_{1z} R_{0x}}{R_{1x}}}
{\frac{ R_{0z} P_{1x} R_{1x} P_{2x} R_{1y} - R_{0z} P_{1x} P_{2y}}{R_{1x} R_{1x} R_{1x} P_{1y} - R_{1x} R_{1x} P_{1x} R_{1y}}
+ \frac{P_{2x}}{R_{1x}}
- P_{1z} \frac{R_{1x} P_{2x} R_{1y} - P_{2y}}
{R_{1x} R_{1x} P_{1y} - R_{1x} P_{1x} R_{1y}}
- P_{2z}}
$$