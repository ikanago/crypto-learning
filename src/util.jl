module Util

using Random

export is_prime, generate_prime

function is_prime(n::BigInt)::Bool
    if n < 1
        return false
    end

    i = BigInt(2)
    while i * i <= n
        if n % i == 0
            return false
        end
        i += 1
    end
    true
end

function generate_prime(bits::Integer)::BigInt
    lower_bound = 1 << (bits - 1)
    upper_bound = 1 << bits
    while true
        n = BigInt(rand(lower_bound:upper_bound))
        if is_prime(n)
            return BigInt(n)
        end
    end
end

end
