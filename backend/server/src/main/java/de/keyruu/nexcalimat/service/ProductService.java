package de.keyruu.nexcalimat.service;

import java.time.LocalDateTime;
import java.util.List;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.transaction.Transactional;

import de.keyruu.nexcalimat.graphql.exception.ProductNotFoundException;
import de.keyruu.nexcalimat.model.Product;
import de.keyruu.nexcalimat.repository.ProductRepository;

@ApplicationScoped
public class ProductService
{
  @Inject
  ProductRepository _productRepository;

  public List<Product> listAll()
  {
    return _productRepository.list("deletedAt IS NULL");
  }

  public Product findById(Long id)
  {
    return _productRepository.findById(id);
  }

  @Transactional
  public Product updateProductPicture(Product product)
  {
    Product dbProduct = _productRepository.findByIdOptional(product.getId()).orElseThrow(ProductNotFoundException::new);

    dbProduct.setPicture(product.getPicture());
    _productRepository.persist(dbProduct);

    return dbProduct;
  }

  @Transactional
  public Product updateProduct(Product product)
  {
    Product dbProduct = _productRepository.findByIdOptional(product.getId()).orElseThrow(ProductNotFoundException::new);

    if (product.getBundleSize() != null)
    {
      dbProduct.setBundleSize(product.getBundleSize());
    }
    if (product.getName() != null)
    {
      dbProduct.setName(product.getName());
    }
    if (product.getPrice() != null)
    {
      dbProduct.setPrice(product.getPrice());
    }
    if (product.getType() != null)
    {
      dbProduct.setType(product.getType());
    }

    _productRepository.persist(dbProduct);

    return dbProduct;
  }

  @Transactional
  public Boolean deleteById(Long id)
  {
    Product product = _productRepository.findByIdOptional(id).orElseThrow(ProductNotFoundException::new);

    product.setDeletedAt(LocalDateTime.now());
    _productRepository.persist(product);

    return Boolean.TRUE;
  }
}
