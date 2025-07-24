# a) 0th: edit title & backdate last-modified-date (→ to be updated)
  (.records[0].title.title.value) |= "Title to be updated"
| (.records[0]."last-modified-date".value) -= 86400000
# b) 1st) duplicate and change title & put-code (→ to be deleted)
| .records += [.records[1]]
| (.records[1]."put-code") |= 999999
| (.records[1].title.title.value) |= "Title to be deleted"
# c) 2nd: remove (→ to be added)
| del(.records[2])
