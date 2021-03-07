using crypto_learning.Elgamel, Test

@testset "Dec(Enc(m)) == m" begin
    m = 3141592
    public_key, private_key = Elgamel.generate_key(32)
    c = Elgamel.encrypt(m, public_key)
    d = Elgamel.decrypt(c, public_key, private_key)
    @test m == d
end