title = "TOML Example"
owner = {
    name = "Tom Preston-Werner"
    organization = "GitHub"
    bio = "GitHub Cofounder & CEO\nLikes tater tots and beer."
    dob = "1979-05-27 07:32:00 +00:00"
}
database = {
    server = "192.168.1.1"
    ports = [8001, 8001, 8002]
    connection_max = 5000
    enabled = true
}
servers = {
    alpha = {
        ip = "10.0.0.1"
        dc = "eqdc10"
    }
    beta = {
        ip = "10.0.0.2"
        dc = "eqdc10"
        country = "中国"
    }
}
clients = {
    data = [["gamma", "delta"], [1, 2]]
    hosts = ["alpha", "omega"]
}
products = [
    {
        name = "Hammer"
        sku = 738594937
    }
    {
        name = "Nail"
        sku = 284758393
        color = "gray"
    }
]