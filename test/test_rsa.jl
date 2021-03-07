using crypto_learning.Rsa, Test

@testset "Dec(Enc(m)) == m" begin
    rand_range = 2:(1<<16)
    m = rand(big.(rand_range))
    public_key, private_key = Rsa.generate_key(32)
    c = Rsa.encrypt(m, public_key)
    d = Rsa.decrypt(c, public_key, private_key)
    @test m == d
end

@testset "Homomorpism of RSA" begin
    public_key, private_key = Rsa.generate_key(32)
    rand_range = 2:(1<<16)
    m1 = rand(big.(rand_range))
    c1 = Rsa.encrypt(m1, public_key)
    m2 = rand(big.(rand_range))
    c2 = Rsa.encrypt(m2, public_key)
    c = c1 * c2
    d = Rsa.decrypt(c, public_key, private_key)
    @test m1 * m2 == d
end
