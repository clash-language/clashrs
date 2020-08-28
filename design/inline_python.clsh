#!/bin/clash
[
    [INLINE Python(bin="/usr/bin/python3") AS pyscript ][
        import socket
        import shlex
        import copy
        import subprocess
        import sys
        global __local_dev_container_address__
        def __local_dev_container_address__():
            return [
                l for l in ([
                    ip for ip in socket.gethostbyname_ex(socket.gethostname())[2]
                    if not ip.startswith("127.")][:1], 
                    [
                        [
                            (
                                s.connect(("8.8.8.8", 53)), 
                                s.getsockname()[0],
                                s.close()
                            ) for s in [
                                socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                            ]
                        ][0][1]
                ]) if l ][0][0]
            ]

        global __connect__
        def __connect__(local_dev_port=18888):
            local_dev_addr = __local_dev_container_address__()
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:
                s.bind((local_dev_addr, local_dev_port))
            except OSError:
                sys.exit(1)
            s.listen(1)
            while True:
                conn, addr = s.accept()
                while True:
                    cmd = conn.recv(1024)
                    cmd = cmd.decode('utf-8')
                    cmd = shlex.split(cmd)
                    if cmd == ['STOP']:
                        conn.close()
                        s.close()
                        sys.exit(0)
                    else:
                        rpc = subprocess.Popen(
                            cmd,
                            stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE,
                        )
                        try:
                            response, _err = b'ERROR\a', b'0'
                            with rpc as proc:
                                response = proc.communicate(timeout=60)
                            while True:
                                if rpc.poll() is not None:
                                    res = response
                                    res += b'\a'
                                    conn.send(res)
                                    break
                                else:
                                    pass
                            break
                        except subprocess.TimeoutExpired:
                            rpc.kill()
                            err = _err + response
                            conn.send(err)
                            conn.close()
        class Namespace:
            @property
            def connect(self, ip_address: str):
                return __connect__(ip_address)
            @property
            def _local_dev_container_address(self):
                return __local_dev_container_address__()
        pyscript = Namespace()
    ]
    [EXPORT INLINE Python AS pyscript USING][ 
        [META ($local_dev_port) FROM ([local_dev_port]=@[0]) -> UDB].connect,
        [META () -> UDB]._local_dev_container_address
    ]
    [END INLINE Python]
]

