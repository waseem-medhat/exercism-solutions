defmodule KitchenCalculator do
  def get_volume({_, vol}), do: vol

  def to_milliliter({unit, vol}) do
    vol_ml = 
      case unit do
        :cup -> vol * 240
        :fluid_ounce -> vol * 30
        :teaspoon -> vol * 5
        :tablespoon -> vol * 15
        :milliliter -> vol
      end

      {:milliliter, vol_ml}
  end

  def from_milliliter({:milliliter, vol}, unit) do
    vol_unit = 
      case unit do
        :cup -> vol / 240
        :fluid_ounce -> vol / 30
        :teaspoon -> vol / 5
        :tablespoon -> vol / 15 
        :milliliter -> vol
      end

      {unit, vol_unit}
  end

  def convert(volume_pair, unit) do
    volume_pair |> to_milliliter() |> from_milliliter(unit)
  end
end
