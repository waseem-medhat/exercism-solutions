defmodule NameBadge do
  def print(id, name, department) do
    id_str = if is_nil(id), do: "", else: "[#{id}] - "
    department_str = if is_nil(department), do: "OWNER", else: department
    
    id_str <> "#{name} - #{String.upcase(department_str)}"
  end
end
