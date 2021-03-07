using crypto_learning.Elgamel, Test

@testset "Dec(Enc(m)) == m" begin
    m = BigInt(3141592)
    public_key, private_key = Elgamel.generate_key(32)
    c = Elgamel.encrypt(m, public_key)
    d = Elgamel.decrypt(c, public_key, private_key)
    @test m == d
end

@testset "Homomorpism of ElGamel" begin
    public_key, private_key = Elgamel.generate_key(32)
    m1 = BigInt(3)
    c1 = Elgamel.encrypt(m1, public_key)
    m2 = BigInt(7)
    c2 = Elgamel.encrypt(m2, public_key)
    (p, _, _) = public_key
    c = (c1[1] * c2[1] % p, c1[2] * c2[2] % p)
    d = Elgamel.decrypt(c, public_key, private_key)
    @test m1 * m2 == d
end
