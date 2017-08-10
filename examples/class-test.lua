function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k,v in pairs(o) do
      if type(k) ~= 'number' then k = '"'..k..'"' end
      s = s .. '['..k..'] = ' .. dump(v) .. ','
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

--print("BUTTON IS:")
--print(dump(button))
--print("BUTTON METATABLE IS:")
--print(dump(getmetatable(button)))

print("Setting button constructor...")
button.connect_signal("new", function(a)
      print("FOO")
      print(string.format("a: %s", type(a)))
    return {b = 700 }
end)

print("Calling button constructor...")
print(dump(button({a = 5})))


--assert(button ~= nil)
--assert(getmetatable(button).__index ~= nil)
--assert(button.__call ~= nil)
--input = {}
--input["a"] = 5
--b = button(input)
--print(string.format("button() returned: %s", tostring(b)))
--assert(b ~= nil)

print("Finished test")
