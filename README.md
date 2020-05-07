# 设置运行权限

    sudo setcap cap_net_admin=eip /home/teng/CLionProjects/tcpimplement/target/release/tcpimplement

# 添加 ip 地址

     sudo ip addr add 10.0.0.1/24 dev tun0

# 启动网卡

     sudo ip link set up dev tun0

# tcp 状态图

           +---------+ ---------\      active OPEN
                                        |  CLOSED |            \    -----------
                                        +---------+<---------\   \   create TCB
                                            |     ^              \   \  snd SYN
                            passive OPEN |     |   CLOSE        \   \
                            ------------ |     | ----------       \   \
                                create TCB  |     | delete TCB         \   \
                                            V     |                      \   \
                                        +---------+            CLOSE    |    \
                                        |  LISTEN |          ---------- |     |
                                        +---------+          delete TCB |     |
                            rcv SYN      |     |     SEND              |     |
                            -----------   |     |    -------            |     V
            +---------+      snd SYN,ACK  /       \   snd SYN          +---------+
            |         |<-----------------           ------------------>|         |
            |   SYN   |                    rcv SYN                     |   SYN   |
            |   RCVD  |<-----------------------------------------------|   SENT  |
            |         |                    snd ACK                     |         |
            |         |------------------           -------------------|         |
            +---------+   rcv ACK of SYN  \       /  rcv SYN,ACK       +---------+
            |           --------------   |     |   -----------
            |                  x         |     |     snd ACK
            |                            V     V
            |  CLOSE                   +---------+
            | -------                  |  ESTAB  |
            | snd FIN                  +---------+
            |                   CLOSE    |     |    rcv FIN
            V                  -------   |     |    -------
            +---------+          snd FIN  /       \   snd ACK          +---------+
            |  FIN    |<-----------------           ------------------>|  CLOSE  |
            | WAIT-1  |------------------                              |   WAIT  |
            +---------+          rcv FIN  \                            +---------+
            | rcv ACK of FIN   -------   |                            CLOSE  |
            | --------------   snd ACK   |                           ------- |
            V        x                   V                           snd FIN V
            +---------+                  +---------+                   +---------+
            |FINWAIT-2|                  | CLOSING |                   | LAST-ACK|
            +---------+                  +---------+                   +---------+
            |                rcv ACK of FIN |                 rcv ACK of FIN |
            |  rcv FIN       -------------- |    Timeout=2MSL -------------- |
            |  -------              x       V    ------------        x       V
                \ snd ACK                 +---------+delete TCB         +---------+
     ------------------------>|TIME WAIT|------------------>| CLOSED  |
                              +---------+                   +---------+
