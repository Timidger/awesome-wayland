function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k,v in pairs(o) do
      if type(k) ~= 'number' then k = '"'..k..'"' end
      s = s .. '['..k..'] = ' .. dump(v) .. ',\n'
    end
    return s .. '} '
  else
    return tostring(o)
  end
end



print("Starting button constructor test")
print(string.format("button is: %s", type(button)))
print(string.format("button.__call is: %s", type(button.__call)))
print(string.format("button.__metatable is: %s", type(getmetatable(button))))

print("Setting button constructor...")

print("Calling button constructor...")
a_button = button({a = 5})
print(dump(a_button))
assert(type(a_button) == "userdata")

print("Finished button constructor test")

print ("Starting awesome test")
print(dump(awesome))
-- NOTE Uncomment to test
--awesome.exec("weston-terminal")

print("Finished test")
