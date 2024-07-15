function Fibonacci(n)
  if n < 2 then
    return n;
  end

  local n1 = Fibonacci(n - 1);
  local n2 = Fibonacci(n - 2);
  return n1 + n2;
end

local result = Fibonacci(30);
print(result);
