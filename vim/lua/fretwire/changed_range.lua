-- Return the union of queued edits in the new buffer's zero-based, end-exclusive rows.
return function(pending_first, pending_last, first, old_last, new_last)
  if pending_first == nil then
    return first, new_last
  end
  local function shifted(row)
    if row >= old_last then
      return row + new_last - old_last
    end
    return math.min(row, first)
  end
  return math.min(first, shifted(pending_first)), math.max(new_last, shifted(pending_last))
end
