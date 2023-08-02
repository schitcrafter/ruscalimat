package de.keyruu.nexcalimat.model.projection;

import de.keyruu.nexcalimat.model.Product;
import de.keyruu.nexcalimat.model.ProductType;
import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class PurchaseCount
{
	private final Product _product;
	private final long _count;

	public PurchaseCount(Product product, long count)
	{
		_product = product;
		_count = count;
	}

	public Product getProduct()
	{
		return _product;
	}

	public long getCount()
	{
		return _count;
	}

	public long getRecommendedPurchaseAmount()
	{
		if (_product.getType() == ProductType.COLD_DRINK)
		{
			return (long)Math.ceil((double)_count / _product.getBundleSize());
		}
		return 0;
	}
}
