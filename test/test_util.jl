using crypto_learning.Util, Test

@testset "Determine an integer is prime number" begin
    @test !Util.is_prime(BigInt(-1))
    @test !Util.is_prime(BigInt(0))
    @test Util.is_prime(BigInt(1))
    @test Util.is_prime(BigInt(2))
    @test !Util.is_prime(BigInt(4))
    @test Util.is_prime(BigInt(7))
end

@testset "Generate prime number" begin
    n = Util.generate_prime(16)
    @test Util.is_prime(n)
end
