module Util

using Random

function is_prime(n::Integer)
    if n < 1
        return false
    end

    i = 2
    while i * i <= n
        if n % i == 0
            return false
        end
        i += 1
    end
    true
end

function generate_prime(bits::Integer)
    lower_bound = 1 << (bits - 1)
    upper_bound = 1 << bits
    while true
        n = rand(lower_bound:upper_bound)
        if is_prime(n)
            return n
        end
    end
end

end